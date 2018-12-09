<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Budget Management System</name>
   <tag></tag>
   <elementGuidId>5b3daff3-739e-4fa5-b58f-9dc9f140463e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
 
  
    Budget Management System
    
      
    
    
      
        
          Balance
        
        
          Daily Input
        
        
          Configuration
        
        
          Balance Simulator
        
        
          Savings History
        
        
          Logout
        
      
    
  


	×
	


	×
	




  
    Current Savings: 0.00  
  
  
    
      
        
          
            
              
            
          
          
        
      
      

      
      
        
          
            
              
            
          
          
        
      
      


      
      
        Visualize Incomes
      
      


      
      
        Visualize Expenses
      
      

      
      
      
        
          Summary
          All year:  0
            Month:  0
        
      
    
    
  




   
  
  return
  
    
      No Period
    
    
      Daily
    
    
      Biweekly
    
    
      Monthly
    
  
  

    
    
      
      
    
    

    
    
      
      
    
    

    
    
      
      
    
    

    
    
      
      
    
    
  




    $(document).ready(function(){

      $('#detail-div').hide();
      // getting date input from
      var date_input=$('input[name=&quot;from&quot;]');

      // getting calendar's container
      var container=$('.bootstrap-iso form').length>0 ? $('.bootstrap-iso form').parent() : &quot;body&quot;;
      // stablishing calendar's input options
      var options={
        format: 'dd/mm/yyyy',
        container: container,
        todayHighlight: true,
        autoclose: true,
      };

      //initialiazing date input from
      date_input.datepicker(options);

      // getting date input to
      var date_input2 = $('input[name=&quot;to&quot;]');
      //initialiazing date input to
      date_input2.datepicker(options)
    })

    function loadData(type, data){

        var listDivName;

        if(type == 1){
          // no period
          listDivName = &quot;list-no-period&quot;;
        }else if(type == 2){
          //daily
          listDivName = &quot;list-daily&quot;;
        }else if(type == 3){
          //biweekly
          listDivName = &quot;list-biweekly&quot;;
        }else{
          // monthly
          listDivName = &quot;list-monthly&quot;;
        }

        var listDiv = document.getElementById(listDivName);
        var fields, value, name, date,l,s,d,h;
        var indexOpeningParenthesis = data.indexOf(&quot;{&quot;);
        var indexClosParenthesis = data.indexOf(&quot;}&quot;);

        for(var i =0; i &lt; data.length; i++){
          if(indexOpeningParenthesis &lt; 0  || indexClosParenthesis &lt; 0 )return;
          var temp = data.substring(indexOpeningParenthesis, indexClosParenthesis+2);
          fields = JSON.parse(temp);
          fields= fields['fields'];
          if(fields == undefined) continue;
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
          indexOpeningParenthesis = data.indexOf(&quot;{&quot;, indexClosParenthesis);
          indexClosParenthesis = data.indexOf(&quot;}&quot;, indexClosParenthesis+1);
        }

    }

    function clearVisualizeDiv(listDivName){
      var listDiv = document.getElementById(listDivName);
      listDiv.innerHTML = &quot;&quot;;
    }

    function goback(){
      $('#main-page').show()
      $('#detail-div').hide();
      clearVisualizeDiv(&quot;list-no-period&quot;);
      clearVisualizeDiv(&quot;list-daily&quot;);
      clearVisualizeDiv(&quot;list-biweekly&quot;);
      clearVisualizeDiv(&quot;list-monthly&quot;);
    }
    function loadIncomes(event){
      $.ajax({
        type: &quot;GET&quot;,
        url:&quot;/visualize/&quot;,
        dataType: &quot;json&quot;,
        async:true, // keep front end responsive to user input
        data:{
          from: $(&quot;#from-date&quot;).val(),
          to: $(&quot;#to-date&quot;).val(),
          type: 1
        },
        success: function(data){
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
        error: function(){


        }
      })
    }

    function loadExpenses(event){
      $.ajax({
        type: &quot;GET&quot;,
        url:&quot;/visualize/&quot;,
        dataType: &quot;json&quot;,
        async:true, // keep front end responsive to user input
        data:{
          csrfmiddlewaretoken:'VfskuQ9qQJjjAWC1aRezmPbS3LEYBTFZdKNrR4L9e2wTUGtavoZyc3yKby8MpW6K',
          from: $(&quot;#from-date&quot;).val(),
          to: $(&quot;#to-date&quot;).val(),
          type: 0
        },
        success: function(data){
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
        error: function(){

        }
      })

    }









 










$( document ).ready(function() {
		// variable &quot;number&quot; sent from backend to highlight navbar tabs
    if(1 == 1){
			$('#li-1').addClass('active');
			// highlight balance (home) tab
		}else if(1 ==2){
			$('#li-2').addClass('active');
			// highlight Daily Input tab
		}else if(1 == 3){
			$('#li-3').addClass('active');
			// highlight Configuration tab
		}else if(1 == 4){
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
				csrfmiddlewaretoken:'VfskuQ9qQJjjAWC1aRezmPbS3LEYBTFZdKNrR4L9e2wTUGtavoZyc3yKby8MpW6K',
				id_concept: $(&quot;#conceptos&quot;).val(),
			},
			success: function(){
				// say to user that concept has been disabled
				document.getElementById('alertMessage').innerHTML = &quot;Concept Disabled, please refresh page&quot;;
				console.log(&quot;succeed&quot;);
			},
			error: function(){
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
				csrfmiddlewaretoken:'VfskuQ9qQJjjAWC1aRezmPbS3LEYBTFZdKNrR4L9e2wTUGtavoZyc3yKby8MpW6K',
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







/html[1]/body[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
   </webElementProperties>
</WebElementEntity>
