<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Budget Management System</name>
   <tag></tag>
   <elementGuidId>8065aaf4-3309-43a5-867d-4f2693bbd5a6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	Budget Management System
	
	

	
  





	
	
	

	
	
	
	
	


#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}



	×
	


	×
	



  
    
     Thanks!
    
      We've emailed you instructions for setting your password, if an account exists with the email you entered.
      You should receive them shortly.
    
    
      
      If you don't receive an email, please make sure you've entered the address you registered with,
      and check your spam folder.
      
    
  
    
      Go Back to Login
    
  




























$( document ).ready(function() {

	$('[data-toggle=&quot;tooltip&quot;]').tooltip();


	$('#detail-div').hide();

		// variable &quot;number&quot; sent from backend to highlight navbar tabs
    if( == 1){
			$('#li-1').addClass('active');
			// highlight balance (home) tab
		}else if( ==2){
			$('#li-2').addClass('active');
			// highlight Daily Input tab
		}else if( == 3){
			$('#li-3').addClass('active');
			// highlight Configuration tab
		}else if( == 4){
			$('#li-4').addClass('active');
			// highlight Balance Simulator tab
		}else{
			$('#li-5').addClass('active');
			// highlight Savings History tab
		}
});

function closeAlertWrong(){
	// used when user wants to close alert dialogue
	$(&quot;#alert_wrong&quot;).hide();
}

function closeAlertSucceed(){
	// used when user wants to close alert dialogue
	$(&quot;#alert_succ&quot;).hide();
}
function deleteConcept(e){
		// $(&quot;#conceptos&quot;).val() gives concept id which must be positive
		if($(&quot;#conceptos&quot;).val() == -1){
			// user has not selected new concept
			$(&quot;#alert_wrong&quot;).show();
			document.getElementById('alertMessageWrong').innerHTML = &quot;Please Select a Concept&quot;;
			return;
		}
		$.ajax({
			type: &quot;GET&quot;,
			url:&quot;/delete_concept/&quot;,
			dataType: &quot;json&quot;,
			data:{
				csrfmiddlewaretoken:'FPlmTzzAtVTDi6q7kZ6tf93BcDDCAo2N5WKQ8QGaWgZIGkORb1jePiKHiPO1uQnd',
				id_concept: $(&quot;#conceptos&quot;).val(),
			},
			success: function(){
				// say to user that concept has been disabled
				$(&quot;#alert_succ&quot;).show();
				document.getElementById('alertMessage').innerHTML = &quot;Concept Disabled, please refresh page&quot;;
				console.log(&quot;succeed&quot;);
			},
			error: function(){
				$(&quot;#alert_wrong&quot;).show();
				document.getElementById('alertMessageWrong').innerHTML = &quot;Something went wrong, please try again&quot;;
				console.log(&quot;error&quot;);

			}
		})
	}


	function deleteDailyInput(e){
		$.ajax({
			type: &quot;GET&quot;,
			url:&quot;/delete_daily/&quot;,
			dataType: &quot;json&quot;,
			async:true, // keep front end responsive to user input
			data:{
				csrfmiddlewaretoken:'FPlmTzzAtVTDi6q7kZ6tf93BcDDCAo2N5WKQ8QGaWgZIGkORb1jePiKHiPO1uQnd',
				id_concept: $(&quot;#conceptos&quot;).val(),
				date: $(&quot;#from-date&quot;).val(),
			},
			success: function(){

				window.location.reload();
				console.log(&quot;succeed&quot;);
			},
			error: function(){
				$(&quot;#alert_wrong&quot;).show();
				document.getElementById('alertMessageWrong').innerHTML = &quot;Something went wrong, please try again&quot;;
				console.log(&quot;error&quot;);

			}
		})
	}

	function changePercentage(e){
		$.ajax({
			type: &quot;GET&quot;,
			url:&quot;/change_percentage/&quot;,
			dataType: &quot;json&quot;,
			async:false, // keep front end responsive to user input
			data:{
				value: $(&quot;#input_percentage&quot;).val(),
			},
			success: function(){
				window.location.reload();
				document.getElementById('alertMessage').innerHTML = &quot;Percentage Changed!&quot;;
				// say to user that concept has been disabled



			},
			error: function(){
				$(&quot;#alert_wrong&quot;).show();
				document.getElementById('alertMessageWrong').innerHTML = &quot;Something went wrong, please try again&quot;;
				console.log(&quot;error&quot;);

			}
		})
	}


	function loadIncomes(event) {
		$.ajax({
			type: &quot;GET&quot;,
			url: &quot;/visualize/&quot;,
			dataType: &quot;json&quot;,
			async: true, // keep front end responsive to user input
			data: {
				from: $(&quot;#from-date&quot;).val(),
				to: $(&quot;#to-date&quot;).val(),
				type: 1
			},
			success: function(data) {
				console.log(data);
				$('#main-page').hide()
				$('#detail-div').show();
				document.getElementById('title-detail').innerHTML = &quot;Incomes&quot;
				document.getElementById(&quot;range-date&quot;).innerHTML = $(&quot;#from-date&quot;).val() + &quot; - &quot; + $(&quot;#to-date&quot;).val();

				no_period = data['unique'];
				daily = data['daily'];
				biweek = data['biweek'];
				monthly = data['monthly'];

				loadData(1, no_period);
				loadData(2, daily);
				loadData(3, biweek);
				loadData(4, monthly);
			},
			error: function() {


			}
		})
	}


	function clearVisualizeDiv(listDivName) {
		var listDiv = document.getElementById(listDivName);
		listDiv.innerHTML = &quot;&quot;;
	}

	function goback() {
		$('#main-page').show()
		$('#detail-div').hide();
		clearVisualizeDiv(&quot;list-no-period&quot;);
		clearVisualizeDiv(&quot;list-daily&quot;);
		clearVisualizeDiv(&quot;list-biweekly&quot;);
		clearVisualizeDiv(&quot;list-monthly&quot;);
	}

	function loadExpenses(event) {
		$.ajax({
			type: &quot;GET&quot;,
			url: &quot;/visualize/&quot;,
			dataType: &quot;json&quot;,
			async: true, // keep front end responsive to user input
			data: {
				csrfmiddlewaretoken: 'FPlmTzzAtVTDi6q7kZ6tf93BcDDCAo2N5WKQ8QGaWgZIGkORb1jePiKHiPO1uQnd',
				from: $(&quot;#from-date&quot;).val(),
				to: $(&quot;#to-date&quot;).val(),
				type: 0
			},
			success: function(data) {
				$('#main-page').hide()
				$('#detail-div').show();
				document.getElementById(&quot;range-date&quot;).innerHTML = $(&quot;#from-date&quot;).val() + &quot; - &quot; + $(&quot;#to-date&quot;).val();
				document.getElementById('title-detail').innerHTML = &quot;Expenses&quot;
				no_period = data['unique'];
				daily = data['daily'];
				biweek = data['biweek'];
				monthly = data['monthly'];

				loadData(1, no_period);
				loadData(2, daily);
				loadData(3, biweek);
				loadData(4, monthly);
			},
			error: function() {
				console.log(&quot;error&quot;);
			}
		})

	}



	function loadData(type, data) {

		var listDivName;

		if (type == 1) {
			// no period
			listDivName = &quot;list-no-period&quot;;
		} else if (type == 2) {
			//daily
			listDivName = &quot;list-daily&quot;;
		} else if (type == 3) {
			//biweekly
			listDivName = &quot;list-biweekly&quot;;
		} else {
			// monthly
			listDivName = &quot;list-monthly&quot;;
		}

		var listDiv = document.getElementById(listDivName);
		var fields, value, name, date, l, s, d, h;
		var indexOpeningParenthesis = data.indexOf(&quot;{&quot;);
		var indexClosParenthesis = data.indexOf(&quot;}&quot;);
		for (var i = 0; i &lt; data.length; i++) {
			if (indexOpeningParenthesis &lt; 0 || indexClosParenthesis &lt; 0) return;
			console.log(&quot;opening: &quot; + indexOpeningParenthesis)
			console.log(&quot;closing: &quot; + indexClosParenthesis)
			var temp = data.substring(indexOpeningParenthesis, indexClosParenthesis + 2);
			console.log(temp);
			fields = JSON.parse(temp);
			fields = fields['fields'];
			if (fields == undefined) continue;
			value = fields['value'];
			name = fields['concept'][0];
			date = fields['date_from'];

			l = document.createElement(&quot;a&quot;);
			l.setAttribute(&quot;href&quot;, &quot;#&quot;);
			l.setAttribute(&quot;class&quot;, &quot;list-group-item list-group-item-action flex-column align-items-start&quot;);

			d = document.createElement(&quot;div&quot;);
			d.setAttribute(&quot;class&quot;, &quot;d-flex w-100 justify-content-between&quot;);

			h = document.createElement(&quot;h5&quot;);
			h.setAttribute(&quot;class&quot;, &quot;mb-1&quot;);
			h.appendChild(document.createTextNode(name));

			s = document.createElement(&quot;small&quot;);
			s.appendChild(document.createTextNode(value));

			d.appendChild(h);
			d.appendChild(s);

			l.appendChild(d);

			s = document.createElement(&quot;small&quot;);
			s.appendChild(document.createTextNode(date));
			l.appendChild(s);

			listDiv.appendChild(l)
			indexOpeningParenthesis = data.indexOf(&quot;{&quot;, indexClosParenthesis+1);
			indexClosParenthesis = data.indexOf(&quot;}&quot;, indexOpeningParenthesis );

		}

	}





/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
