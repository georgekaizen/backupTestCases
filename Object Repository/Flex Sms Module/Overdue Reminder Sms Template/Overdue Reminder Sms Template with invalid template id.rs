<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>overdue sms reminder for customers</description>
   <name>Overdue Reminder Sms Template with invalid template id</name>
   <tag></tag>
   <elementGuidId>198ca571-c57f-4c57-8141-00563c3758a3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;recipients&quot;,
      &quot;value&quot;: &quot;254704258258&quot;
    },
    {
      &quot;name&quot;: &quot;template_id&quot;,
      &quot;value&quot;: &quot;85&quot;
    },
    {
      &quot;name&quot;: &quot;first_name&quot;,
      &quot;value&quot;: &quot;kaizen&quot;
    },
    {
      &quot;name&quot;: &quot;product_name&quot;,
      &quot;value&quot;: &quot;iron box&quot;
    },
    {
      &quot;name&quot;: &quot;outlet&quot;,
      &quot;value&quot;: &quot;Friesen Group&quot;
    },
    {
      &quot;name&quot;: &quot;overdue_amount&quot;,
      &quot;value&quot;: &quot;2200&quot;
    },
    {
      &quot;name&quot;: &quot;amount_paid&quot;,
      &quot;value&quot;: &quot;1000&quot;
    },
    {
      &quot;name&quot;: &quot;balance&quot;,
      &quot;value&quot;: &quot;3200&quot;
    },
    {
      &quot;name&quot;: &quot;till_number&quot;,
      &quot;value&quot;: &quot;695071&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://176.58.103.227:8086/api/send_sms</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
